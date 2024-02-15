import * as THREE from "three";
import { ThreeElements, useFrame } from "@react-three/fiber";
import { useEffect, useRef, useState } from "react";

export default function Box() {
    const meshRef = useRef<THREE.Mesh>(null!);
    const lineSegmentsRef = useRef<THREE.LineSegments>(null!);

    const [hovered, setHover] = useState(false);

    // rotate x -- pitch
    const keyWPressedRef = useRef<boolean>(false);
    const [keyWPressed, setKeyWPressed] = useState(keyWPressedRef.current);
    const keySPressedRef = useRef<boolean>(false);
    const [keySPressed, setKeySPressed] = useState(keySPressedRef.current);

    // rotate z -- roll
    const keyAPressedRef = useRef<boolean>(false);
    const [keyAPressed, setKeyAPressed] = useState(keyAPressedRef.current);
    const keyDPressedRef = useRef<boolean>(false);
    const [keyDPressed, setKeyDPressed] = useState(keyDPressedRef.current);

    // rotate y -- yaw
    const keyQPressedRef = useRef<boolean>(false);
    const [keyQPressed, setKeyQPressed] = useState(keyQPressedRef.current);
    const keyEPressedRef = useRef<boolean>(false);
    const [keyEPressed, setKeyEPressed] = useState(keyEPressedRef.current);

    useEffect(() => {
        const keyDown = (event: KeyboardEvent) => {
            event.preventDefault();
            updateKeyStatus(event.code, true);
        };
        const keyUp = (event: KeyboardEvent) => {
            event.preventDefault();
            updateKeyStatus(event.code, false);
        };
        document.addEventListener("keydown", keyDown);
        document.addEventListener("keyup", keyUp);
        return () => {
            document.removeEventListener("keydown", keyDown);
            document.removeEventListener("keyup", keyUp);
        };
    }, []);

    const updateKeyStatus = (key: string, status: boolean) => {
        switch (key) {
            case "KeyW":
                keyWPressedRef.current = status;
                setKeyWPressed(keyWPressedRef.current);
                break;
            case "KeyS":
                keySPressedRef.current = status;
                setKeySPressed(keySPressedRef.current);
                break;
            case "KeyA":
                keyAPressedRef.current = status;
                setKeyAPressed(keyAPressedRef.current);
                break;
            case "KeyD":
                keyDPressedRef.current = status;
                setKeyDPressed(keyDPressedRef.current);
                break;
            case "KeyQ":
                keyQPressedRef.current = status;
                setKeyQPressed(keyQPressedRef.current);
                break;
            case "KeyE":
                keyEPressedRef.current = status;
                setKeyEPressed(keyEPressedRef.current);
                break;
            default:
                break;
        }
    };

    useFrame((state, delta) => {
        if (keyWPressed) {
            meshRef.current.rotation.x += delta;
            lineSegmentsRef.current.rotation.x += delta;
        }
        if (keySPressed) {
            meshRef.current.rotation.x -= delta;
            lineSegmentsRef.current.rotation.x -= delta;
        }
        if (keyAPressed) {
            meshRef.current.rotation.z += delta;
            lineSegmentsRef.current.rotation.z += delta;
        }
        if (keyDPressed) {
            meshRef.current.rotation.z -= delta;
            lineSegmentsRef.current.rotation.z -= delta;
        }
        if (keyQPressed) {
            meshRef.current.rotation.y += delta;
            lineSegmentsRef.current.rotation.y += delta;
        }
        if (keyEPressed) {
            meshRef.current.rotation.y -= delta;
            lineSegmentsRef.current.rotation.y -= delta;
        }
    });

    return (
        <>
            <mesh position={[0, 0, 0]} ref={meshRef} onPointerOver={() => setHover(true)} onPointerOut={() => setHover(false)}>
                <boxGeometry args={[2, 1, 1]} />
                <meshStandardMaterial color={hovered ? "hotpink" : "orange"} />
            </mesh>
            <lineSegments ref={lineSegmentsRef}>
                <edgesGeometry args={[new THREE.BoxGeometry(2, 1, 1)]} />
                <lineBasicMaterial color={0x000000} />
            </lineSegments>
        </>
    );
}
