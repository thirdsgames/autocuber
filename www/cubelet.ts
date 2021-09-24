import * as THREE from 'three';

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

// Rounds each element of this quaternion to the nearest(ish) value in the list:
// -1, -sqrt(2)/2, -1/2, 0, 1/2, sqrt(2)/2, 1
// Squared:
// -1, -1/2, -1/4, 0, 1/4, 1/2, 1
function roundQuat(quat: THREE.Quaternion) {
    const r = (x: number): number => {
        const x2 = x * Math.abs(x);
        if (x2 < -0.75) {
            return -1;
        }
        if (x2 < -0.375) {
            return -Math.SQRT1_2;
        }
        if (x2 < -0.125) {
            return -1 / 2;
        }
        if (x2 < 0.125) {
            return 0;
        }
        if (x2 < 0.375) {
            return 1 / 2;
        }
        if (x2 < 0.75) {
            return Math.SQRT1_2;
        }
        return 1;
    };

    quat.set(r(quat.x), r(quat.y), r(quat.z), r(quat.w));
}

function easeInOutSine(x: number): number {
    return -(Math.cos(Math.PI * x) - 1) / 2;
}

export default class Cubelet {
    root: THREE.Object3D = new THREE.Object3D();

    faces: THREE.Mesh[] = [];

    // The position in the cube's coordinate system that the piece resides currently.
    // Can be scaled if the mesh itself is scaled.
    // Used as a "target position" for animations.
    logicalPosition: THREE.Vector3 = new THREE.Vector3();

    prevLogicalPosition: THREE.Vector3 = new THREE.Vector3();

    // A piece's rotation as a quaternion.
    // Rounded using roundQuat.
    // Used as a "target rotation" for animations.
    logicalRotation: THREE.Quaternion = new THREE.Quaternion();

    prevLogicalRotation: THREE.Quaternion = new THREE.Quaternion();

    axis: THREE.Vector3 = new THREE.Vector3();

    animationTime: number = 0;

    scale: number;

    // The cubelet should be a position on the cube, represented as x-y-z integer coords.
    // The cube's core has coordinates (0, 0, 0).
    // The cubelet faces are ordered F R U B L D.
    constructor(
        scene: THREE.Scene,
        x: number,
        y: number,
        z: number,
        faceScale: number,
        scale: number
    ) {
        this.scale = scale;

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
            mesh.scale.setScalar(faceScale);
            mesh.parent = this.root;
            this.root.add(mesh);
            this.faces.push(mesh);
        }
        scene.add(this.root);

        this.faces[0].position.add(new THREE.Vector3(0, 0, faceScale * 0.5));

        this.faces[1].position.add(new THREE.Vector3(faceScale * 0.5, 0, 0));
        this.faces[1].rotateY(Math.PI * 0.5);

        this.faces[2].position.add(new THREE.Vector3(0, faceScale * 0.5, 0));
        this.faces[2].rotateX(Math.PI * -0.5);

        this.faces[3].position.add(new THREE.Vector3(0, 0, faceScale * -0.5));
        this.faces[3].rotateY(Math.PI);

        this.faces[4].position.add(new THREE.Vector3(faceScale * -0.5, 0, 0));
        this.faces[4].rotateY(Math.PI * -0.5);

        this.faces[5].position.add(new THREE.Vector3(0, faceScale * -0.5, 0));
        this.faces[5].rotateX(Math.PI * 0.5);

        this.set(
            new THREE.Vector3(0.0, 0.0, 0.0),
            new THREE.Quaternion(),
            new THREE.Vector3(0, 0, -1)
        );
    }

    // Interpolates visually to the given position and rotation.
    // The cubelets are visually rotating along the given axis.
    // If no axis is given, naive interpolation is used.
    set(position: THREE.Vector3, rotation: THREE.Quaternion, axis: THREE.Vector3) {
        this.animationTime = 0.0;

        this.prevLogicalPosition.copy(this.root.position).divideScalar(this.scale);
        this.logicalPosition.copy(position);
        this.logicalPosition.round();

        this.prevLogicalRotation.copy(this.root.quaternion);
        this.logicalRotation.copy(rotation);
        roundQuat(this.logicalRotation);

        this.axis.copy(axis);
    }

    update(delta: number) {
        this.animationTime += delta;
        const animSpeed = 0.5;
        const t = easeInOutSine(
            Math.min(Math.max(this.animationTime, 0.0), 1 / animSpeed) * animSpeed
        );

        if (this.axis.x === 0 && this.axis.y === 0 && this.axis.z === 0) {
            this.root.position
                .copy(this.prevLogicalPosition)
                .lerp(this.logicalPosition, t)
                .multiplyScalar(this.scale);
        } else {
            // Use a custom position lerp that preserves the side length when
            // rotating through the given axis.
            // We use cylindrical polar coordinates for this purpose.
            // The z axis is the provided axis in `set`.
            // First, rotate our coordinate system such that the z axis is upwards.
            // A random up direction is chosen, it is not important.
            const rotZ = new THREE.Matrix4().lookAt(
                new THREE.Vector3(),
                this.axis,
                new THREE.Vector3(1, 2, 4).normalize()
            );
            const invRotZ = new THREE.Matrix4().copy(rotZ).invert();

            const start = new THREE.Vector3().copy(this.prevLogicalPosition).applyMatrix4(invRotZ);
            const end = new THREE.Vector3().copy(this.logicalPosition).applyMatrix4(invRotZ);

            const rhoStart = Math.sqrt(start.x * start.x + start.y * start.y);
            const rhoEnd = Math.sqrt(end.x * end.x + end.y * end.y);

            const phiStart = Math.atan2(start.y, start.x);
            const phiEnd = Math.atan2(end.y, end.x);

            const angleLerp = (a: number, b: number) => {
                if (b - a < -Math.PI) {
                    return (1 - t) * a + t * (b + 2 * Math.PI);
                }
                if (b - a > Math.PI) {
                    return (1 - t) * a + t * (b - 2 * Math.PI);
                }
                return (1 - t) * a + t * b;
            };

            const z = (1 - t) * start.z + t * end.z;
            const rho = angleLerp(rhoStart, rhoEnd);
            const phi = angleLerp(phiStart, phiEnd);

            this.root.position
                .copy(
                    new THREE.Vector3(rho * Math.cos(phi), rho * Math.sin(phi), z).applyMatrix4(
                        rotZ
                    )
                )
                .multiplyScalar(this.scale);
        }

        this.root.quaternion.copy(this.prevLogicalRotation).slerp(this.logicalRotation, t);
    }
}
