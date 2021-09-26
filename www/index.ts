import * as THREE from 'three';
import * as wasm from 'autocuber';

import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import Cube, { Face } from './cube';

// UI initialisation

const scene = new THREE.Scene();

const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.z = 2;

const renderer = new THREE.WebGLRenderer({
    antialias: true,
});
renderer.setClearColor(0x181818, 1);
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const controls = new OrbitControls(camera, renderer.domElement);
controls.enablePan = false;
controls.enableZoom = false;

const cube = new Cube(scene);

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

let prevFrameTime = performance.now();
function animate() {
    const now = performance.now();
    const delta = (now - prevFrameTime) / 1000;
    prevFrameTime = now;

    requestAnimationFrame(animate);

    controls.update();
    cube.update(delta);

    render();
}

animate();

// Create the move div
{
    const move = document.getElementById('move');
    const moveHeader = document.createElement('h1');
    moveHeader.innerText = 'Move';
    move.appendChild(moveHeader);
    const table = document.createElement('table');

    const faces: Face[] = ['F', 'R', 'U', 'B', 'L', 'D', 'M', 'E', 'S'];
    faces.forEach((face) => {
        const row = document.createElement('tr');

        const types: [string, number][] = [
            ['', 1],
            ["'", -1],
            ['2', 2],
            ['w', 1],
            ["w'", -1],
            ['w2', 2],
        ];
        types.forEach(([name, rotations]) => {
            let innerText = face + name;
            let realFace = face;
            if (name.includes('w')) {
                if (['M', 'E', 'S'].includes(face)) {
                    // Replace Mw, Ew, Sw with x, y, z rotations.
                    realFace = face.replace('M', 'x').replace('E', 'y').replace('S', 'z') as Face;
                    innerText = realFace + name.substr(1);
                } else {
                    // Replace Fw with f, etc.
                    realFace = face.toLowerCase() as Face;
                    innerText = realFace + name.substr(1);
                }
            }

            const td = document.createElement('td');
            const button = document.createElement('button');
            button.addEventListener('click', (_ev) => {
                if (!cube.animating) {
                    cube.move({ face: realFace, rotations });
                }
            });
            button.innerText = innerText;
            td.appendChild(button);
            row.appendChild(td);
        });

        table.appendChild(row);
    });

    move.appendChild(table);
}

// Create the history div
{
    const history = document.getElementById('history');
    const historyHeader = document.createElement('h1');
    historyHeader.innerText = 'History';
    history.appendChild(historyHeader);

    const button = document.createElement('button');
    button.addEventListener('click', (_ev) => {
        if (!cube.animating) {
            cube.reset();
        }
    });
    button.innerText = 'Reset';
    history.appendChild(button);
}

// WASM

const universe = wasm.init();
console.log(universe);

wasm.greet();
universe.free();
