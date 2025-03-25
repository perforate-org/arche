import type { Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";
import { createEffect, createContext, useContext } from "solid-js";
import type { Component, JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { canisterId, createActor, backend } from "../declarations/backend";
import { matchResult } from "../utils/result";
import { router } from "../main";
import { pushAlert } from "./alert";

export const AuthContext = createContext<AuthStoreType>();

export type AuthStoreType = {
  isAuthenticated: boolean;
  isInitializing: boolean;
  login: () => void;
  logout: () => Promise<void>;
  authClient: AuthClient | null;
  identity: Identity | null;
  principal: Principal | null;
  backend: typeof backend;
};

export function getIdentityProvider() {
  let idpProvider;
  // Safeguard against server rendering
  if (typeof window !== "undefined") {
    const isLocal = import.meta.env.DFX_NETWORK !== "ic";
    // Safari does not support localhost subdomains
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
    if (isLocal && isSafari) {
      idpProvider = `http://localhost:4943/?canisterId=${import.meta.env.CANISTER_ID_INTERNET_IDENTITY}`;
    } else if (isLocal) {
      idpProvider = `http://${import.meta.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
    }
  }
  return idpProvider;
}

export const defaultOptions = {
  /**
   *  @type {import("@dfinity/auth-client").AuthClientCreateOptions}
   */
  createOptions: {
    idleOptions: {
      // Set to true if you do not want idle functionality
      disableIdle: true,
    },
  },
  /**
   * @type {import("@dfinity/auth-client").AuthClientLoginOptions}
   */
  loginOptions: {
    identityProvider: getIdentityProvider(),
    // 7 days in milliseconds
    maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000),
  },
};

/**
 *
 * @param options - Options for the AuthClient
 * @param {AuthClientCreateOptions} options.createOptions - Options for the AuthClient.create() method
 * @param {AuthClientLoginOptions} options.loginOptions - Options for the AuthClient.login() method
 * @returns
 */
export function useAuthClient(
  routerProp: typeof router,
  options = defaultOptions,
): AuthStoreType {
  const [store, setStore] = createStore<AuthStoreType>({
    isAuthenticated: false,
    isInitializing: true,
    login: () => {
      store.authClient?.login({
        ...options.loginOptions,
        onSuccess: async () => {
          await updateClient(store.authClient);
          let isRegistered = await store.backend.is_registered();
          if (!isRegistered) {
            let response = await store.backend.register_user();
            matchResult(response, {
              ok: () => {
                pushAlert({
                  type: "success",
                  message:
                    "Welcome to Arche! Your account has been successfully created.",
                });
                setTimeout(() => {
                  pushAlert({
                    type: "info",
                    message:
                      "Set up your account: update your ID and name if desired.",
                  });
                }, 1500);
                routerProp.navigate({
                  to: "/account/settings",
                });
              },
              err: (error) => {
                console.error(error);
              },
            });
          }
        },
      });
    },
    logout: async () => {
      await store.authClient?.logout();
      await updateClient(store.authClient);
      if (routerProp.matchRoute("/account")) {
        routerProp.navigate({
          to: "/",
        });
      }
    },
    authClient: null,
    identity: null,
    principal: null,
    backend: backend,
  });

  createEffect(() => {
    // Initialize AuthClient
    AuthClient.create(options.createOptions).then(async (client) => {
      updateClient(client);
      setStore("isInitializing", false);
    });
  }, []);

  async function updateClient(client: AuthClient | null) {
    if (!client) {
      console.log("AuthClient is not initialized");
      return;
    }

    const isAuthenticated = await client.isAuthenticated();
    setStore("isAuthenticated", isAuthenticated);

    const identity = client.getIdentity();
    setStore("identity", identity);

    const principal = identity.getPrincipal();
    setStore("principal", principal);

    setStore("authClient", client);

    const actor = createActor(canisterId, {
      agentOptions: {
        identity,
      },
    });

    setStore("backend", actor);
  }

  return store;
}

export const AuthProvider: Component<{
  children: JSXElement;
  router: typeof router;
}> = (props) => {
  const store = useAuthClient(router);

  return (
    <AuthContext.Provider value={store}>{props.children}</AuthContext.Provider>
  );
};

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within an AuthProvider");
  return context;
}
