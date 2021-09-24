import * as THREE from 'three';
import { NumberKeyframeTrack, Vector3 } from 'three';

const plane = new THREE.PlaneGeometry(1, 1, 1, 1);
const colours = {
    R: 0xdd4444,
    G: 0x22cc55,
    B: 0x2244dd,
    W: 0xdddddd,
    Y: 0xdddd22,
    O: 0xee8822,
    K: 0x080808,
};
const materials = Object.fromEntries(
    Object.entries(colours).map(([name, color]) => [name, new THREE.MeshBasicMaterial({ color })])
);

export default class Cubelet {
    root: THREE.Object3D = new THREE.Object3D();

    faces: THREE.Mesh[] = [];

    // The cubelet should be a position on the cube, represented as x-y-z integer coords.
    // The cube's core has coordinates (0, 0, 0).
    // The cubelet faces are ordered F R U B L D.
    constructor(scene: THREE.Scene, x: number, y: number, z: number, scale: number) {
        for (let i = 0; i < 6; i += 1) {
            let material = materials.K;

            // Give each face the right colour.
            switch (i) {
                case 0:
                    if (z === 1) {
                        material = materials.G;
                    }
                    break;
                case 1:
                    if (x === 1) {
                        material = materials.R;
                    }
                    break;
                case 2:
                    if (y === 1) {
                        material = materials.W;
                    }
                    break;
                case 3:
                    if (z === -1) {
                        material = materials.B;
                    }
                    break;
                case 4:
                    if (x === -1) {
                        material = materials.O;
                    }
                    break;
                case 5:
                    if (y === -1) {
                        material = materials.Y;
                    }
                    break;
                // no default
            }

            const mesh = new THREE.Mesh(plane, material);
            mesh.scale.setScalar(scale);
            mesh.parent = this.root;
            this.root.add(mesh);
            this.faces.push(mesh);
        }
        scene.add(this.root);

        this.faces[0].position.add(new THREE.Vector3(0, 0, scale * 0.5));

        this.faces[1].position.add(new THREE.Vector3(scale * 0.5, 0, 0));
        this.faces[1].rotateY(Math.PI * 0.5);

        this.faces[2].position.add(new THREE.Vector3(0, scale * 0.5, 0));
        this.faces[2].rotateX(Math.PI * -0.5);

        this.faces[3].position.add(new THREE.Vector3(0, 0, scale * -0.5));
        this.faces[3].rotateY(Math.PI);

        this.faces[4].position.add(new THREE.Vector3(scale * -0.5, 0, 0));
        this.faces[4].rotateY(Math.PI * -0.5);

        this.faces[5].position.add(new THREE.Vector3(0, scale * -0.5, 0));
        this.faces[5].rotateX(Math.PI * 0.5);

        this.setPosition(new THREE.Vector3(0.0, 0.0, 0.0));
    }

    setPosition(position: THREE.Vector3) {
        this.root.position.set(position.x, position.y, position.z);
    }
}
