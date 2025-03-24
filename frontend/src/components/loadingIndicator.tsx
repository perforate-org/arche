import { createSignal, createEffect } from "solid-js";
import { useIsFetching } from "@tanstack/solid-query";

const threshold = 1500; // ms
const fadeOutDelay = 300; // ms
const transitionDuration = 150; // ms

export function GlobalLoadingIndicator() {
  const isFetching = useIsFetching();
  const [shouldShow, setShouldShow] = createSignal(false);
  const [isVisible, setIsVisible] = createSignal(false);
  let showTimeout: number;
  let fadeTimeout: number;
  let transitionTimeout: number;
  let progressRef: HTMLProgressElement | undefined;

  createEffect(() => {
    if (isFetching()) {
      clearTimeout(showTimeout);
      clearTimeout(fadeTimeout);
      clearTimeout(transitionTimeout);

      setIsVisible(true);
      if (progressRef) {
        progressRef.removeAttribute("value");
      }
      showTimeout = setTimeout(() => setShouldShow(true), threshold);
    } else {
      clearTimeout(showTimeout);

      if (progressRef) {
        progressRef.value = 1;
      }

      fadeTimeout = setTimeout(() => {
        setShouldShow(false);
      }, fadeOutDelay);

      transitionTimeout = setTimeout(
        () => setIsVisible(false),
        fadeOutDelay + transitionDuration,
      );
    }
  });

  return (
    <div class="pointer-events-none fixed top-0 z-50 flex w-screen items-center justify-center pt-1">
      {isVisible() && (
        <progress
          ref={progressRef}
          class={`progress h-1 w-10 transition-all duration-${transitionDuration} opacity-0 ${shouldShow() ? "w-20 opacity-50" : ""}`}
          aria-label="Loading"
        />
      )}
    </div>
  );
}
