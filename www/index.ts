import * as THREE from 'three';
import * as wasm from 'autocuber';
import { RotationType, Axis, Move, inverse } from 'autocuber';

import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import Cube from './cube';

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

    const faces = ['F', 'R', 'U', 'B', 'L', 'D', 'M', 'E', 'S'];
    faces.forEach((face) => {
        const row = document.createElement('tr');

        const types: [string, number][] = [
            ['', RotationType.Normal],
            ["'", RotationType.Inverse],
            ['2', RotationType.Double],
            ['w', RotationType.Normal],
            ["w'", RotationType.Inverse],
            ['w2', RotationType.Double],
        ];
        types.forEach(([name, rotationType]) => {
            let innerText = face + name;
            let axis: Axis;
            let startDepth: number;
            let endDepth: number;
            const wide = name.includes('w');

            switch (face) {
                case 'F':
                case 'S':
                case 'B':
                    axis = Axis.FB;
                    break;
                case 'R':
                case 'M':
                case 'L':
                    axis = Axis.RL;
                    break;
                case 'U':
                case 'E':
                case 'D':
                    axis = Axis.UD;
                    break;
                // no default
            }

            switch (face) {
                case 'F':
                case 'R':
                case 'U':
                    startDepth = 0;
                    endDepth = wide ? 2 : 1;
                    break;
                case 'M':
                case 'E':
                case 'S':
                    if (wide) {
                        // Replace Mw, Ew, Sw with x, y, z rotations.
                        startDepth = 0;
                        endDepth = 3;
                    } else {
                        startDepth = 1;
                        endDepth = 2;
                    }
                    break;
                case 'B':
                case 'L':
                case 'D':
                    startDepth = wide ? 1 : 2;
                    endDepth = 3;
                    break;
                // no default
            }

            let realRotationType: RotationType;
            switch (face) {
                case 'B':
                case 'L':
                case 'D':
                case 'M':
                case 'E':
                    realRotationType = inverse(rotationType);
                    break;
                default:
                    realRotationType = rotationType;
            }

            if (wide) {
                if (['M', 'E', 'S'].includes(face)) {
                    // Replace Mw, Ew, Sw with x, y, z rotations.
                    const realFace = face.replace('M', 'x').replace('E', 'y').replace('S', 'z');
                    innerText = realFace + name.substr(1);
                } else {
                    // Replace Fw with f, etc.
                    const realFace = face.toLowerCase();
                    innerText = realFace + name.substr(1);
                }
            }

            const td = document.createElement('td');
            const button = document.createElement('button');

            button.addEventListener('click', (_ev) => {
                if (!cube.animating) {
                    cube.move(Move.new(axis, realRotationType, startDepth, endDepth));
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
let currentAlgStep = -1;
{
    const history = document.getElementById('history');
    const historyHeader = document.createElement('h1');
    historyHeader.innerText = 'History';
    history.appendChild(historyHeader);

    {
        const button = document.createElement('button');
        button.addEventListener('click', (_ev) => {
            if (!cube.animating) {
                cube.reset();
                currentAlgStep = -1;
            }
        });
        button.innerText = 'Reset';
        history.appendChild(button);
    }

    const inner = document.createElement('div');
    inner.id = 'history-action';
    history.appendChild(inner);
}

function processHistory(moveSequence: Array<Move>) {
    currentAlgStep = -1;
    // Adds listeners to each history element.
    // When clicked, they navigate to that move in the sequence.
    const historyAction = document.getElementById('history-action');
    const moves = Array.from(historyAction.getElementsByClassName('history-move'));
    moves.forEach((element, i) => {
        element.addEventListener('click', (_ev) => {
            if (!cube.animating) {
                if (i > currentAlgStep) {
                    cube.performAlg(moveSequence.slice(currentAlgStep + 1, i + 1));
                    currentAlgStep = i;
                } else if (i < currentAlgStep) {
                    const slice = moveSequence
                        .slice(i + 1, currentAlgStep + 1)
                        .reverse()
                        .map((mv) => mv.clone_move().inverse());
                    cube.performAlg(slice);
                    currentAlgStep = i;
                }
            }
        });
    });
}

// WASM

const universe = wasm.init();
console.log(universe);

processHistory(wasm.action_to_div());

universe.free();
