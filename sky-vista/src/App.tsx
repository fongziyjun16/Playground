import { useEffect } from "react";
import { appWindow } from "@tauri-apps/api/window";

export default function App() {
    useEffect(() => {
        document.addEventListener("mousedown", onMouseDown);
        return () => {
            document.removeEventListener("mousedown", onMouseDown);
        };
    }, []);

    const onMouseDown = (event: MouseEvent) => {
        const target = event.target as HTMLElement;
        if (target.getAttribute("drag-region") === "true" && event.buttons === 1 && event.detail === 1) {
            appWindow.startDragging().catch(console.log);
        }
    };

    return (
        <div className="w-screen h-screen bg-transparent">
            <div drag-region="true" className="w-full h-[64px]">
                Sky Vista
            </div>
        </div>
    );
}
