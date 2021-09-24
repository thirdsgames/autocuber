import * as THREE from 'three';

import Cubelet from './cubelet';

function pieceIndex(x: number, y: number, z: number): number {
    return x * 9 + y * 3 + z;
}

export default class Cube {
    pieces: Record<number, Cubelet> = {};

    constructor(scene: THREE.Scene) {
        const scale = 0.3;
        const faceScale = scale * 0.96;

        [-1, 0, 1].forEach((x) =>
            [-1, 0, 1].forEach((y) =>
                [-1, 0, 1].forEach((z) => {
                    const cubelet = new Cubelet(scene, x, y, z, faceScale);
                    cubelet.setPosition(new THREE.Vector3(x * scale, y * scale, z * scale));
                    this.pieces[pieceIndex(x, y, z)] = cubelet;
                })
            )
        );
    }
}
