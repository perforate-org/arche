import { createSignal, onMount } from "solid-js";
import { Account } from "./header/account";
import { Link } from "@tanstack/solid-router";

const threshold = 24;

export function Header() {
  const [isVisible, setIsVisible] = createSignal(true);
  const [lastScrollY, setLastScrollY] = createSignal(0);

  const handleScroll = () => {
    const currentScrollY = window.scrollY;

    if (Math.abs(currentScrollY - lastScrollY()) > threshold) {
      setIsVisible(currentScrollY < lastScrollY() || currentScrollY <= 0);
      setLastScrollY(currentScrollY);
    }
  };

  onMount(() => {
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  });

  return (
    <nav
      class="fixed top-0 right-0 left-0 z-2 flex h-12 flex-row border-b-[0.5px] border-slate-300 bg-stone-100 px-4 py-1 transition-transform duration-300 md:px-8 md:px-16 dark:border-slate-700 dark:bg-stone-900"
      style={{
        transform: isVisible() ? "translateY(0)" : "translateY(-100%)",
      }}
    >
      <div class="container flex flex-row pr-2 md:pr-4 md:pl-1">
        <Logo />
      </div>
      <div class="container flex items-center justify-end px-2 md:px-4">
        <Account />
      </div>
    </nav>
  );
}

function Logo() {
  return (
    <Link
      class={`active:text-base-content/50 px-2 py-1 transition duration-50 md:px-3`}
      to={"/"}
    >
      <h1 class="text-2xl font-bold">Arche</h1>
    </Link>
  );
}
