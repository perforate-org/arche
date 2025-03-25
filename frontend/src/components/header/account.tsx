import { Link } from "@tanstack/solid-router";
import { useAuth } from "../../contexts/auth";
import type { AuthStoreType } from "../../contexts/auth";

export function Account() {
  const auth = useAuth();

  return (
    <>
      {!auth.isAuthenticated ? (
        <LoginButton auth={auth} />
      ) : (
        <AccountDropdown auth={auth} />
      )}
    </>
  );
}

function LoginButton({ auth }: { auth: AuthStoreType }) {
  const handleLogin = () => {
    auth.login();
  };

  return (
    <button
      class="group btn btn-xs btn-soft text-base-content/50 hover:text-base-content flex gap-2 transition"
      onClick={handleLogin}
    >
      <img
        src="/images/infinity-black.png"
        alt="Internet Computer Infinity Logo"
        class="opacity-50 transition group-hover:opacity-100 dark:invert"
        width="16"
        height="16"
      />
      Login with Internet Identity
    </button>
  );
}

function AccountDropdown({ auth }: { auth: AuthStoreType }) {
  return (
    <div class="dropdown dropdown-end">
      <div
        tabindex="0"
        role="button"
        class="btn btn-xs btn-soft text-base-content/75 hover:text-base-content m-1 transition"
      >
        Account
      </div>
      <ul
        tabindex="0"
        class="menu dropdown-content rounded-box bg-base-100 z-1 flex w-52 gap-2 border-[0.5px] border-slate-300 p-2 text-xs dark:border-slate-700"
      >
        <li>
          <Link class="px-2 py-1.5" to={"/account/paper-list"}>
            Manage Papers
          </Link>
        </li>
        <li>
          <Link class="px-2 py-1.5" to={"/account/settings"}>
            Account Settings
          </Link>
        </li>
        <div class="divider pointer-events-none -my-2"></div>
        <li>
          <LogoutButton auth={auth} />
        </li>
      </ul>
    </div>
  );
}

function LogoutButton({ auth }: { auth: AuthStoreType }) {
  const handleLogout = () => {
    auth.logout();
  };

  return (
    <button class="btn btn-xs btn-soft transition" onClick={handleLogout}>
      Logout
    </button>
  );
}
