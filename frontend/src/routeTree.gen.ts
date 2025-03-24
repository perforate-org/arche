/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as PapersImport } from './routes/papers'
import { Route as AboutImport } from './routes/about'
import { Route as IndexImport } from './routes/index'
import { Route as UsersUserIdImport } from './routes/users/$userId'
import { Route as AccountSettingsImport } from './routes/account/settings'
import { Route as AccountPaperListImport } from './routes/account/paper-list'
import { Route as AbsPaperIdImport } from './routes/abs/$paperId'

// Create/Update Routes

const PapersRoute = PapersImport.update({
  id: '/papers',
  path: '/papers',
  getParentRoute: () => rootRoute,
} as any)

const AboutRoute = AboutImport.update({
  id: '/about',
  path: '/about',
  getParentRoute: () => rootRoute,
} as any)

const IndexRoute = IndexImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const UsersUserIdRoute = UsersUserIdImport.update({
  id: '/users/$userId',
  path: '/users/$userId',
  getParentRoute: () => rootRoute,
} as any)

const AccountSettingsRoute = AccountSettingsImport.update({
  id: '/account/settings',
  path: '/account/settings',
  getParentRoute: () => rootRoute,
} as any)

const AccountPaperListRoute = AccountPaperListImport.update({
  id: '/account/paper-list',
  path: '/account/paper-list',
  getParentRoute: () => rootRoute,
} as any)

const AbsPaperIdRoute = AbsPaperIdImport.update({
  id: '/abs/$paperId',
  path: '/abs/$paperId',
  getParentRoute: () => rootRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/solid-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/about': {
      id: '/about'
      path: '/about'
      fullPath: '/about'
      preLoaderRoute: typeof AboutImport
      parentRoute: typeof rootRoute
    }
    '/papers': {
      id: '/papers'
      path: '/papers'
      fullPath: '/papers'
      preLoaderRoute: typeof PapersImport
      parentRoute: typeof rootRoute
    }
    '/abs/$paperId': {
      id: '/abs/$paperId'
      path: '/abs/$paperId'
      fullPath: '/abs/$paperId'
      preLoaderRoute: typeof AbsPaperIdImport
      parentRoute: typeof rootRoute
    }
    '/account/paper-list': {
      id: '/account/paper-list'
      path: '/account/paper-list'
      fullPath: '/account/paper-list'
      preLoaderRoute: typeof AccountPaperListImport
      parentRoute: typeof rootRoute
    }
    '/account/settings': {
      id: '/account/settings'
      path: '/account/settings'
      fullPath: '/account/settings'
      preLoaderRoute: typeof AccountSettingsImport
      parentRoute: typeof rootRoute
    }
    '/users/$userId': {
      id: '/users/$userId'
      path: '/users/$userId'
      fullPath: '/users/$userId'
      preLoaderRoute: typeof UsersUserIdImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexRoute
  '/about': typeof AboutRoute
  '/papers': typeof PapersRoute
  '/abs/$paperId': typeof AbsPaperIdRoute
  '/account/paper-list': typeof AccountPaperListRoute
  '/account/settings': typeof AccountSettingsRoute
  '/users/$userId': typeof UsersUserIdRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexRoute
  '/about': typeof AboutRoute
  '/papers': typeof PapersRoute
  '/abs/$paperId': typeof AbsPaperIdRoute
  '/account/paper-list': typeof AccountPaperListRoute
  '/account/settings': typeof AccountSettingsRoute
  '/users/$userId': typeof UsersUserIdRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexRoute
  '/about': typeof AboutRoute
  '/papers': typeof PapersRoute
  '/abs/$paperId': typeof AbsPaperIdRoute
  '/account/paper-list': typeof AccountPaperListRoute
  '/account/settings': typeof AccountSettingsRoute
  '/users/$userId': typeof UsersUserIdRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths:
    | '/'
    | '/about'
    | '/papers'
    | '/abs/$paperId'
    | '/account/paper-list'
    | '/account/settings'
    | '/users/$userId'
  fileRoutesByTo: FileRoutesByTo
  to:
    | '/'
    | '/about'
    | '/papers'
    | '/abs/$paperId'
    | '/account/paper-list'
    | '/account/settings'
    | '/users/$userId'
  id:
    | '__root__'
    | '/'
    | '/about'
    | '/papers'
    | '/abs/$paperId'
    | '/account/paper-list'
    | '/account/settings'
    | '/users/$userId'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexRoute: typeof IndexRoute
  AboutRoute: typeof AboutRoute
  PapersRoute: typeof PapersRoute
  AbsPaperIdRoute: typeof AbsPaperIdRoute
  AccountPaperListRoute: typeof AccountPaperListRoute
  AccountSettingsRoute: typeof AccountSettingsRoute
  UsersUserIdRoute: typeof UsersUserIdRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexRoute: IndexRoute,
  AboutRoute: AboutRoute,
  PapersRoute: PapersRoute,
  AbsPaperIdRoute: AbsPaperIdRoute,
  AccountPaperListRoute: AccountPaperListRoute,
  AccountSettingsRoute: AccountSettingsRoute,
  UsersUserIdRoute: UsersUserIdRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/about",
        "/papers",
        "/abs/$paperId",
        "/account/paper-list",
        "/account/settings",
        "/users/$userId"
      ]
    },
    "/": {
      "filePath": "index.tsx"
    },
    "/about": {
      "filePath": "about.tsx"
    },
    "/papers": {
      "filePath": "papers.tsx"
    },
    "/abs/$paperId": {
      "filePath": "abs/$paperId.tsx"
    },
    "/account/paper-list": {
      "filePath": "account/paper-list.tsx"
    },
    "/account/settings": {
      "filePath": "account/settings.tsx"
    },
    "/users/$userId": {
      "filePath": "users/$userId.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
