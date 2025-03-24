import {
  createSignal,
  createContext,
  useContext,
  createEffect,
  onMount,
  For,
  type Accessor,
  type Setter,
  type JSXElement,
} from "solid-js";
import { Info } from "./alert/info";
import { Success } from "./alert/success";
import { Warning } from "./alert/warning";
import { Error } from "./alert/error";

const alertDismissDelay = 5000; // ms

export type AlertType =
  | { type: "warning"; message: string }
  | { type: "success"; message: string }
  | { type: "info"; message: string }
  | { type: "error"; message: string };

const [alertsState, setAlertsState] = createSignal<AlertType[]>([]);

const AlertsContext = createContext<Accessor<AlertType[]>>(alertsState);
const SetAlertsContext = createContext<Setter<AlertType[]>>(setAlertsState);

export function pushAlert(alert: AlertType) {
  const setAlertsState = useContext(SetAlertsContext);
  setAlertsState((prev) => [...prev, alert]);
}

export function AlertStack() {
  const alerts = useContext(AlertsContext);
  const setAlerts = useContext(SetAlertsContext);

  function handleRemove(alert: AlertType) {
    setAlerts((prev) => prev.filter((a) => a !== alert));
  }

  const AlertItem = (props: { alert: AlertType; onRemove: () => void }) => {
    const [entered, setEntered] = createSignal(false);
    const [exiting, setExiting] = createSignal(false);
    const [container, setContainer] = createSignal<HTMLDivElement | null>(null);

    // Trigger the entry animation on mount.
    onMount(() => {
      requestAnimationFrame(() => {
        setEntered(true);
      });
      const el = container();
      if (el) {
        el.style.height = el.scrollHeight + "px";
      }
    });

    // Schedule the exit animation after the alertThreshold.
    createEffect(() => {
      const timer = setTimeout(() => {
        setExiting(true);
        const el = container();
        if (el) {
          // Set the current height explicitly before animating to 0.
          el.style.height = el.scrollHeight + "px";
          requestAnimationFrame(() => {
            el.style.height = "0px";
          });
        }
        // Wait for the exit animation to complete (300ms) before removal.
        setTimeout(() => {
          props.onRemove();
        }, 300);
      }, alertDismissDelay);
      return () => clearTimeout(timer);
    });

    const animationClasses = () => {
      if (!entered() || exiting()) {
        return "opacity-0 -translate-y-4";
      }
      return "opacity-100 translate-y-0";
    };

    return (
      <div
        ref={setContainer}
        class="pointer-events-auto overflow-hidden transition-[height] duration-300"
      >
        <div
          class={"transform transition-all duration-300 " + animationClasses()}
        >
          {props.alert.type === "warning" ? (
            <Warning message={props.alert.message} />
          ) : props.alert.type === "success" ? (
            <Success message={props.alert.message} />
          ) : props.alert.type === "info" ? (
            <Info message={props.alert.message} />
          ) : props.alert.type === "error" ? (
            <Error message={props.alert.message} />
          ) : null}
        </div>
      </div>
    );
  };

  return (
    <div class="pointer-events-none fixed top-0 left-0 z-50 w-full space-y-2 p-4">
      <For each={alerts()}>
        {(alert) => (
          <AlertItem alert={alert} onRemove={() => handleRemove(alert)} />
        )}
      </For>
    </div>
  );
}

export function AlertsProvider({ children }: { children: JSXElement }) {
  return (
    <AlertsContext.Provider value={alertsState}>
      <SetAlertsContext.Provider value={setAlertsState}>
        {children}
      </SetAlertsContext.Provider>
    </AlertsContext.Provider>
  );
}
