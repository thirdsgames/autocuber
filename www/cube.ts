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

export type Face =
    | 'F'
    | 'f'
    | 'R'
    | 'r'
    | 'U'
    | 'u'
    | 'B'
    | 'b'
    | 'L'
    | 'l'
    | 'D'
    | 'd'
    | 'M'
    | 'E'
    | 'S'
    | 'x'
    | 'y'
    | 'z';
interface Move {
    face: Face;
    rotations: number;
}

function parseMove(s: string): Move {
    const move = { face: s[0] as Face, rotations: 1 };
    for (let i = 0; i < s.length; i += 1) {
        if (s[i] === "'") {
            move.rotations *= -1;
        } else if (s[i] === '2') {
            move.rotations *= 2;
        }
    }
    return move;
}

function parseAlg(s: string): Move[] {
    return s.split(' ').map(parseMove);
}

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
        switch (move.face) {
            case 'F':
                this.move_any((_x, _y, z) => z === 1, new THREE.Vector3(0, 0, 1), move.rotations);
                break;
            case 'f':
                this.move_any((_x, _y, z) => z !== -1, new THREE.Vector3(0, 0, 1), move.rotations);
                break;
            case 'z':
                this.move_any((_x, _y, _z) => true, new THREE.Vector3(0, 0, 1), move.rotations);
                break;
            case 'S':
                this.move_any((_x, _y, z) => z === 0, new THREE.Vector3(0, 0, 1), move.rotations);
                break;
            case 'R':
                this.move_any((x, _y, _z) => x === 1, new THREE.Vector3(1, 0, 0), move.rotations);
                break;
            case 'r':
                this.move_any((x, _y, _z) => x !== -1, new THREE.Vector3(1, 0, 0), move.rotations);
                break;
            case 'x':
                this.move_any((_x, _y, _z) => true, new THREE.Vector3(1, 0, 0), move.rotations);
                break;
            case 'U':
                this.move_any((_x, y, _z) => y === 1, new THREE.Vector3(0, 1, 0), move.rotations);
                break;
            case 'u':
                this.move_any((_x, y, _z) => y !== -1, new THREE.Vector3(0, 1, 0), move.rotations);
                break;
            case 'y':
                this.move_any((_x, _y, _z) => true, new THREE.Vector3(0, 1, 0), move.rotations);
                break;
            case 'B':
                this.move_any((_x, _y, z) => z === -1, new THREE.Vector3(0, 0, -1), move.rotations);
                break;
            case 'b':
                this.move_any((_x, _y, z) => z !== 1, new THREE.Vector3(0, 0, -1), move.rotations);
                break;
            case 'L':
                this.move_any((x, _y, _z) => x === -1, new THREE.Vector3(-1, 0, 0), move.rotations);
                break;
            case 'l':
                this.move_any((x, _y, _z) => x !== 1, new THREE.Vector3(-1, 0, 0), move.rotations);
                break;
            case 'M':
                this.move_any((x, _y, _z) => x === 0, new THREE.Vector3(-1, 0, 0), move.rotations);
                break;
            case 'D':
                this.move_any((_x, y, _z) => y === -1, new THREE.Vector3(0, -1, 0), move.rotations);
                break;
            case 'd':
                this.move_any((_x, y, _z) => y !== 1, new THREE.Vector3(0, -1, 0), move.rotations);
                break;
            case 'E':
                this.move_any((_x, y, _z) => y === 0, new THREE.Vector3(0, -1, 0), move.rotations);
                break;
            // no default
        }
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
                // Wait for the last move's animation to finish.
                this.animating = false;
            }, 334);
        };

        if (!this.animating) {
            this.animating = true;
            animate();
        }
    }
}
