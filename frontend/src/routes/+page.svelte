<script lang="ts">
	import { Simulation } from 'simulation-wasm';
	import { onMount } from 'svelte';
	import { devicePixelRatio, innerHeight, innerWidth } from 'svelte/reactivity/window';

	let simulation: any = null;
	let canvas: HTMLCanvasElement;
	let context2D: CanvasRenderingContext2D | null = null;
	let ready = false;
	let canvasHeight: number;
	let canvasWidth: number;

	const viewportScale = devicePixelRatio.current || 1;

	onMount(async () => {
		simulation = new Simulation();

		context2D = canvas.getContext('2d');
		ready = true;

		canvasHeight = (innerHeight.current || 300) * viewportScale;
		canvasWidth = (innerWidth.current || 400) * viewportScale;

		redraw();
	});

	const drawTriangle = (x: number, y: number, size: number, rotation: number) => {
		if (!context2D) return;

		context2D.beginPath();
		context2D.moveTo(x - Math.sin(rotation) * size * 1.5, y + Math.cos(rotation) * size * 1.5);
		context2D.lineTo(
			x - Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size,
			y + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size
		);
		context2D.lineTo(
			x - Math.sin(rotation + (4.0 / 3.0) * Math.PI) * size,
			y + Math.cos(rotation + (4.0 / 3.0) * Math.PI) * size
		);
		context2D.lineTo(x - Math.sin(rotation) * size * 1.5, y + Math.cos(rotation) * size * 1.5);

		context2D.fillStyle = 'rgb(255,255,255)';
		context2D.fill();
		context2D.stroke();
	};

	const drawCircle = (x: number, y: number, radius: number) => {
		if (!context2D) return;

		context2D.beginPath();

		context2D.arc(x, y, radius, 0, 2.0 * Math.PI);

		context2D.fillStyle = 'rgb(0, 255, 128)';
		context2D.fill();
	};

	const redraw = () => {
		if (!context2D || !simulation) return;

		context2D.clearRect(0, 0, canvasWidth, canvasHeight);

		simulation.step();

		const world = simulation.world();

		for (const food of world.foods) {
			drawCircle(food.x * canvasWidth, food.y * canvasHeight, (0.01 / 2.0) * canvasWidth);
		}

		for (const animal of world.animals) {
			drawTriangle(
				animal.x * canvasWidth,
				animal.y * canvasHeight,
				0.01 * canvasWidth,
				animal.rotation
			);
		}

		requestAnimationFrame(redraw);
	};
</script>

<canvas
	bind:this={canvas}
	id="viewport"
	width={canvasWidth}
	height={canvasHeight}
	style="background-color: steelblue;"
></canvas>

<button
	disabled={!ready}
	onclick={() => {
		console.log(simulation.train());
	}}>Train</button
>

{redraw()}
