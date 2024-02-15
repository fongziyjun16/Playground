import { Canvas } from "@react-three/fiber";
import Box from "./Box";
import { OrbitControls, Stats } from "@react-three/drei";

export default function ObjectCanvas() {
    return (
        <>
            <Canvas>
                <ambientLight intensity={Math.PI / 2} />
                <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} decay={0} intensity={Math.PI} />
                <pointLight position={[-10, -10, -10]} decay={0} intensity={Math.PI} />
                <Box />
                <OrbitControls />
                <axesHelper args={[12]} />
                <gridHelper args={[4, 4, 0xff000, "teal"]} />
                <Stats />
            </Canvas>
        </>
    );
}
