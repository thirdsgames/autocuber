import * as THREE from 'three';

import Cubelet from './cubelet';

export default class Cube {
    centres: Cubelet[] = [];

    constructor(scene: THREE.Scene) {
        const scale = 0.3;
        const faceScale = scale * 0.96;

        [
            [0, 0, 1],
            [0, 0, -1],
            [0, 1, 0],
            [0, -1, 0],
            [1, 0, 0],
            [-1, 0, 0],
        ].forEach(([x, y, z]) => {
            const cubelet = new Cubelet(scene, x, y, z, faceScale);
            cubelet.setPosition(new THREE.Vector3(x * scale, y * scale, z * scale));
            this.centres.push(cubelet);
        });
    }
}
