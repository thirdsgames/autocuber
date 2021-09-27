import * as THREE from 'three';
import { Axis, Move, RotationType } from 'autocuber';

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

    piecesByPosition: Record<number, Cubelet> = {};

    constructor(scene: THREE.Scene) {
        [-1, 0, 1].forEach((x) =>
            [-1, 0, 1].forEach((y) =>
                [-1, 0, 1].forEach((z) => {
                    if (x === 0 && y === 0 && z === 0) {
                        // Don't create the core as a piece
                        return;
                    }
                    const cubelet = new Cubelet(scene, x, y, z, faceScale, scale);
                    cubelet.set(
                        new THREE.Vector3(x, y, z),
                        new THREE.Quaternion(),
                        new THREE.Vector3(0, 0, 0)
                    );
                    this.pieces[pieceIndex(x, y, z)] = cubelet;
                    this.piecesByPosition[pieceIndex(x, y, z)] = cubelet;
                })
            )
        );
    }

    update(delta: number) {
        Object.values(this.pieces).forEach((cubelet) => {
            // const [x, y, z] = indexToPiece(parseInt(n, 10));
            cubelet.update(delta);
        });
    }

    // Rotations is an integer, treated as a multiple of pi/2 clockwise rotations
    move_any(
        spec: (x: number, y: number, z: number) => boolean,
        axis: THREE.Vector3,
        rotations: number
    ) {
        const quat = new THREE.Quaternion().setFromAxisAngle(
            axis,
            -Math.sign(rotations) * (Math.PI / 2)
        );

        const updates: [[number, number, number], Cubelet][] = [];
        [-1, 0, 1].forEach((x) => {
            [-1, 0, 1].forEach((y) => {
                [-1, 0, 1].forEach((z) => {
                    if (x === 0 && y === 0 && z === 0) return;
                    if (!spec(x, y, z)) return;

                    const piece = this.piecesByPosition[pieceIndex(x, y, z)];
                    const pos = new THREE.Vector3(x, y, z);
                    for (let i = 0; i < Math.abs(rotations); i += 1) {
                        pos.applyQuaternion(quat).round();
                        piece.set(
                            pos,
                            new THREE.Quaternion().copy(quat).multiply(piece.logicalRotation),
                            axis
                        );
                        piece.update(0.001);
                    }
                    updates.push([[pos.x, pos.y, pos.z], piece]);
                });
            });
        });

        updates.forEach(([[x, y, z], piece]) => {
            this.piecesByPosition[pieceIndex(x, y, z)] = piece;
        });
    }

    move(move: Move) {
        const axis = move.axis as Axis;

        let rotations;
        switch (move.rotationType) {
            case RotationType.Normal:
                rotations = 1;
                break;
            case RotationType.Inverse:
                rotations = -1;
                break;
            case RotationType.Double:
                rotations = 2;
                break;
            // no default
        }

        switch (axis) {
            case Axis.FB:
                this.move_any(
                    (_x, _y, z) => {
                        const depth = 1 - z;
                        return move.startDepth <= depth && depth < move.endDepth;
                    },
                    new THREE.Vector3(0, 0, 1),
                    rotations
                );
                break;
            case Axis.RL:
                this.move_any(
                    (x, _y, _z) => {
                        const depth = 1 - x;
                        return move.startDepth <= depth && depth < move.endDepth;
                    },
                    new THREE.Vector3(1, 0, 0),
                    rotations
                );
                break;
            case Axis.UD:
                this.move_any(
                    (_x, y, _z) => {
                        const depth = 1 - y;
                        return move.startDepth <= depth && depth < move.endDepth;
                    },
                    new THREE.Vector3(0, 1, 0),
                    rotations
                );
                break;
            // no default
        }

        move.free();
    }

    animating: boolean = false;

    reset() {
        const animate = () => {
            Object.entries(this.pieces).forEach(([position, cubelet]) => {
                const [x, y, z] = indexToPiece(parseInt(position, 10));
                cubelet.set(
                    new THREE.Vector3(x, y, z),
                    new THREE.Quaternion(),
                    new THREE.Vector3()
                );
            });
            Object.assign(this.piecesByPosition, this.pieces);
            setTimeout(() => {
                // Wait for the animation to finish.
                this.animating = false;
            }, 334);
        };

        if (!this.animating) {
            this.animating = true;
            animate();
        }
    }

    n: number = 0;

    performAlg(alg: Move[]) {
        const animate = () => {
            this.move(alg[this.n]);
            if (this.n + 1 < alg.length) {
                setTimeout(() => animate(), 334);
                this.n += 1;
            } else {
                this.n = 0;
                setTimeout(() => {
                    // Wait for the last move's animation to finish.
                    this.animating = false;
                }, 334);
            }
        };

        if (!this.animating) {
            this.animating = true;
            animate();
        }
    }
}
