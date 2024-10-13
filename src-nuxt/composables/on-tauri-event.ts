import { listen, type EventCallback, type EventName, type Options } from "@tauri-apps/api/event";

export function onTauriEvent<T>(event: EventName, handler: EventCallback<T>, options?: Options) {
  const unlisten = listen(event, handler, options);
  onBeforeUnmount(unlisten);
}
