import * as wasm from 'autocuber';

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

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
  Object.entries(colours).map(([name, color]) => [name, new THREE.MeshBasicMaterial({ color })]),
);

class Cubelet {
  root: THREE.Object3D = new THREE.Object3D();

  faces: THREE.Mesh[] = [];

  // The cubelet should be a string of two or three colours (e.g. "RY" for red F and yellow U).
  // The cubelet faces are ordered F R U B L D.
  constructor(scene: THREE.Scene, cubelet: string) {
    for (let i = 0; i < 6; i += 1) {
      let material;
      if (cubelet[i] !== undefined) {
        material = materials[cubelet[i]];
      } else {
        material = materials.K;
      }
      const mesh = new THREE.Mesh(plane, material);
      mesh.parent = this.root;
      scene.add(mesh);
      this.faces.push(mesh);
    }

    this.faces[0].position.add(new THREE.Vector3(0, 0, 0.5));

    this.faces[1].position.add(new THREE.Vector3(0.5, 0, 0));
    this.faces[1].rotateY(Math.PI * 0.5);

    this.faces[2].position.add(new THREE.Vector3(0, 0.5, 0));
    this.faces[2].rotateX(Math.PI * -0.5);

    this.faces[3].position.add(new THREE.Vector3(0, 0, -0.5));
    this.faces[3].rotateY(Math.PI);

    this.faces[4].position.add(new THREE.Vector3(-0.5, 0, 0));
    this.faces[4].rotateY(Math.PI * -0.5);

    this.faces[5].position.add(new THREE.Vector3(0, -0.5, 0));
    this.faces[5].rotateX(Math.PI * 0.5);

    this.setPosition(new THREE.Vector3(0.0, 0.0, 0.0));
  }

  setPosition(position: THREE.Vector3) {
    this.root.position.set(position.x, position.y, position.z);
  }
}

const scene = new THREE.Scene();

const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.z = 2;

const renderer = new THREE.WebGLRenderer();
renderer.setClearColor(0x181818, 1);
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const controls = new OrbitControls(camera, renderer.domElement);
controls.enablePan = false;
controls.enableZoom = false;

const cubelet = new Cubelet(scene, 'GRWBOY');

function render() {
  renderer.render(scene, camera);
}

function onWindowResize() {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
  render();
}
window.addEventListener('resize', onWindowResize, false);

function animate() {
  requestAnimationFrame(animate);

  // cube.rotation.x += 0.01;
  // cube.rotation.y += 0.01;

  controls.update();

  render();
}

animate();

const universe = wasm.init();
console.log(universe);

wasm.greet();
universe.free();
