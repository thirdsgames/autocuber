import * as THREE from 'three';

import Cubelet from './cubelet';

function pieceIndex(x: number, y: number, z: number): number {
    return x * 9 + y * 3 + z;
}

function mod(n: number, m: number): number {
    return ((n % m) + m) % m;
}

function indexToPiece(n: number): [number, number, number] {
    let n2 = n;

    let z = mod(n2, 3);
    n2 -= z;
    n2 /= 3;
    if (z === 2) {
        z = -1;
        n2 += 1;
    }

    let y = mod(n2, 3);
    n2 -= y;
    n2 /= 3;
    if (y === 2) {
        y = -1;
        n2 += 1;
    }

    return [n2, y, z];
}

const scale = 0.3;
const faceScale = scale * 0.96;

export default class Cube {
    pieces: Record<number, Cubelet> = {};

    constructor(scene: THREE.Scene) {
        [-1, 0, 1].forEach((x) =>
            [-1, 0, 1].forEach((y) =>
                [-1, 0, 1].forEach((z) => {
                    if (x === 0 && y === 0 && z === 0) {
                        // Don't create the core as a piece
                        return;
                    }
                    const cubelet = new Cubelet(scene, x, y, z, faceScale);
                    cubelet.set(
                        new THREE.Vector3(x * scale, y * scale, z * scale),
                        new THREE.Quaternion(),
                        new THREE.Vector3(0, 0, 0)
                    );
                    this.pieces[pieceIndex(x, y, z)] = cubelet;
                })
            )
        );
    }

    update(delta: number) {
        Object.entries(this.pieces).forEach(([n, cubelet]) => {
            // const [x, y, z] = indexToPiece(parseInt(n, 10));
            cubelet.update(delta);
        });
    }

    click() {
        console.log('Click!');

        [-1, 0, 1].forEach((x) =>
            [-1, 0, 1].forEach((y) => {
                const piece = this.pieces[pieceIndex(x, y, 1)];
                piece.set(
                    new THREE.Vector3(y * scale, -x * scale, scale),
                    new THREE.Quaternion().setFromAxisAngle(
                        new THREE.Vector3(0, 0, 1),
                        -Math.PI / 2
                    ),
                    new THREE.Vector3(0, 0, -1)
                );
            })
        );
    }
}
