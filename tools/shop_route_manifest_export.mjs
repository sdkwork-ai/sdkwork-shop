#!/usr/bin/env node

import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const checkMode = process.argv.includes('--check');
const HTTP_METHODS = ['get', 'post', 'put', 'patch', 'delete'];
const METHOD_VARIANTS = { get: 'Get', post: 'Post', put: 'Put', patch: 'Patch', delete: 'Delete' };
const TARGETS = [
  {
    surface: 'app-api',
    authority: 'sdkwork-shop-app-api',
    family: 'sdkwork-shop-app-sdk',
    modules: [
      {
        capability: 'shop',
        packageName: 'sdkwork-routes-shop-app-api',
        crateImport: 'sdkwork_routes_shop_app_api',
        source: 'apis/app-api/shop/shop-app-api.openapi.json',
        manifestTarget: 'sdks/_route-manifests/app-api/sdkwork-routes-shop-app-api.route-manifest.json',
        rustTarget: 'crates/sdkwork-routes-shop-app-api/src/http_route_manifest.rs',
        rustFunction: 'app_route_manifest',
      },
    ],
  },
  {
    surface: 'backend-api',
    authority: 'sdkwork-shop-backend-api',
    family: 'sdkwork-shop-backend-sdk',
    modules: [
      {
        capability: 'shop',
        packageName: 'sdkwork-routes-shop-backend-api',
        crateImport: 'sdkwork_routes_shop_backend_api',
        source: 'apis/backend-api/shop/shop-backend-api.openapi.json',
        manifestTarget: 'sdks/_route-manifests/backend-api/sdkwork-routes-shop-backend-api.route-manifest.json',
        rustTarget: 'crates/sdkwork-routes-shop-backend-api/src/http_route_manifest.rs',
        rustFunction: 'backend_route_manifest',
      },
      {
        capability: 'merchandise',
        packageName: 'sdkwork-routes-merchandise-backend-api',
        crateImport: 'sdkwork_routes_merchandise_backend_api',
        source: '../sdkwork-merchandise/apis/backend-api/merchandise/shop-backend-api.merchandise.openapi.json',
        manifestTarget: '../sdkwork-merchandise/sdks/_route-manifests/backend-api/sdkwork-routes-merchandise-backend-api.route-manifest.json',
        external: true,
      },
    ],
  },
];

function stableJson(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

function requiredString(value, label) {
  if (typeof value !== 'string' || value.trim() === '') throw new Error(`${label} must not be empty`);
  return value;
}

function operations(document, target, module, seenRoutes, seenOperationIds) {
  if (document.info?.['x-sdkwork-owner'] !== 'sdkwork-shop') {
    throw new Error(`${module.source} document owner mismatch`);
  }
  if (document.info?.['x-sdkwork-api-authority'] !== target.authority) {
    throw new Error(`${module.source} document authority mismatch`);
  }
  const result = [];
  for (const [operationPath, pathItem] of Object.entries(document.paths ?? {})) {
    for (const method of HTTP_METHODS) {
      const operation = pathItem?.[method];
      if (!operation) continue;
      const label = `${method.toUpperCase()} ${operationPath}`;
      if (operation['x-sdkwork-owner'] !== 'sdkwork-shop') {
        throw new Error(`${label} owner mismatch`);
      }
      if (operation['x-sdkwork-api-authority'] !== target.authority) {
        throw new Error(`${label} authority mismatch`);
      }
      if (operation['x-sdkwork-source-route-crate'] !== module.packageName) {
        throw new Error(`${label} route crate mismatch`);
      }
      if (operation['x-sdkwork-source'] !== `${module.packageName}:${operationPath}`) {
        throw new Error(`${label} source mismatch`);
      }
      if (operation['x-sdkwork-api-surface'] !== target.surface) {
        throw new Error(`${label} API surface mismatch`);
      }
      requiredString(operation['x-sdkwork-permission'], `${label} x-sdkwork-permission`);
      const operationId = requiredString(operation.operationId, `${label} operationId`);
      const routeKey = `${method.toUpperCase()} ${operationPath}`;
      if (seenRoutes.has(routeKey)) throw new Error(`route collision: ${routeKey}`);
      if (seenOperationIds.has(operationId)) throw new Error(`operationId collision: ${operationId}`);
      seenRoutes.add(routeKey);
      seenOperationIds.add(operationId);
      result.push({ method, operationPath, operation });
    }
  }
  return result;
}

function renderRust(routes, functionName) {
  const entries = routes.map(({ method, operationPath, operation }) => [
    '    HttpRoute::dual_token(',
    `        HttpMethod::${METHOD_VARIANTS[method]},`,
    `        ${JSON.stringify(operationPath)},`,
    '        "shop",',
    `        ${JSON.stringify(operation.operationId)},`,
    '    ),',
  ].join('\n'));
  return [
    'use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};',
    '',
    'const HTTP_ROUTES: &[HttpRoute] = &[',
    ...entries,
    '];',
    '',
    `pub fn ${functionName}() -> HttpRouteManifest {`,
    '    HttpRouteManifest::new(HTTP_ROUTES)',
    '}',
    '',
  ].join('\n');
}

function routeManifest(routes, target, module) {
  return {
    schemaVersion: 1,
    kind: 'sdkwork.route.manifest',
    packageName: module.packageName,
    surface: target.surface,
    owner: 'sdkwork-shop',
    domain: 'commerce',
    capability: module.capability,
    apiAuthority: target.authority,
    sdkFamily: target.family,
    prefix: target.surface === 'app-api' ? '/app/v3/api' : '/backend/v3/api',
    source: {
      crateRoot: `crates/${module.packageName}`,
      crateImport: module.crateImport,
    },
    routes: routes.map(({ method, operationPath, operation }) => ({
      method: method.toUpperCase(),
      path: operationPath,
      operationId: operation.operationId,
      tags: operation.tags ?? ['shop'],
      auth: {
        mode: operation['x-sdkwork-auth-mode'] ?? 'dual-token',
        required: true,
        permission: operation['x-sdkwork-permission'],
      },
      ownership: { owner: 'sdkwork-shop', apiAuthority: target.authority },
      source: { file: operation['x-sdkwork-source'] },
      requestContext: 'WebRequestContext',
      apiSurface: target.surface,
    })),
  };
}

/**
 * The gate checks route-set synchronization, not formatting: the manifest
 * files are Rust sources `cargo fmt` owns, and its line wrapping may differ
 * from this exporter's emitter without the route set having drifted. Both
 * sides are compared with whitespace collapsed, so a rustfmt pass can never
 * read as a route drift.
 */
function normalizeRouteContent(value) {
  return value.replace(/\s+/gu, '').replace(/,\)/gu, ')');
}

function synchronize(relativePath, content) {
  const targetPath = path.join(root, relativePath);
  const current = existsSync(targetPath) ? readFileSync(targetPath, 'utf8') : '';
  if (checkMode && normalizeRouteContent(current) !== normalizeRouteContent(content)) {
    if (process.env.DEBUG_MANIFEST_SYNC) {
      const a = normalizeRouteContent(current);
      const b = normalizeRouteContent(content);
      for (let i = 0; i < Math.max(a.length, b.length); i += 1) {
        if (a[i] !== b[i]) {
          console.error('DEBUG first diff at', i);
          console.error('disk :', JSON.stringify(a.slice(i - 60, i + 80)));
          console.error('emit :', JSON.stringify(b.slice(i - 60, i + 80)));
          break;
        }
      }
    }
    throw new Error(`${relativePath} is not synchronized`);
  }
  if (!checkMode && normalizeRouteContent(current) !== normalizeRouteContent(content)) {
    mkdirSync(path.dirname(targetPath), { recursive: true });
    writeFileSync(targetPath, content, 'utf8');
  }
}

function verifyExternal(relativePath, content) {
  const targetPath = path.join(root, relativePath);
  const current = existsSync(targetPath) ? readFileSync(targetPath, 'utf8') : '';
  if (current !== content) {
    throw new Error(`${relativePath} is not synchronized; materialize it from its owning workspace`);
  }
}

try {
  let routeCount = 0;
  let moduleCount = 0;
  for (const target of TARGETS) {
    const seenRoutes = new Set();
    const seenOperationIds = new Set();
    for (const module of target.modules) {
      const document = JSON.parse(readFileSync(path.join(root, module.source), 'utf8'));
      const routes = operations(document, target, module, seenRoutes, seenOperationIds);
      const manifest = stableJson(routeManifest(routes, target, module));
      routeCount += routes.length;
      moduleCount += 1;
      if (module.external) {
        verifyExternal(module.manifestTarget, manifest);
        continue;
      }
      synchronize(module.manifestTarget, manifest);
      synchronize(module.rustTarget, renderRust(routes, module.rustFunction));
    }
  }
  if (!checkMode) {
    // The emitter writes one call per route and `cargo fmt` owns the line
    // wrapping, so materialization ends inside rustfmt's jurisdiction: the
    // written manifests must be rustfmt-clean or `format:check` would answer
    // every materialization with a formatting diff.
    execFileSync('cargo', ['fmt', '--all'], { cwd: root, stdio: 'inherit' });
  }
  process.stdout.write(
    `[shop_route_manifest_export] ${checkMode ? 'check passed' : 'materialized'} `
      + `(${routeCount} routes across ${moduleCount} capability modules)\n`,
  );
} catch (error) {
  process.stderr.write(`[shop_route_manifest_export] ${error instanceof Error ? error.message : String(error)}\n`);
  process.exit(1);
}
