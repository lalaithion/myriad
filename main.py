import numpy as np
from simulation import simulation
import time

import random
import sdl2.ext
import sdl2.sdlgfx

sim = simulation()

num_types = 20
num_particles = 10000

forces = np.float32(np.random.uniform(-1, 1, (num_types, num_types)))
types = np.int8(np.random.randint(0, num_types, num_particles))
x_pos = np.float32(np.random.uniform(-3, 3, num_particles))
y_pos = np.float32(np.random.uniform(-3, 3, num_particles))
x_vel = np.float32(np.zeros(num_particles))
y_vel = np.float32(np.zeros(num_particles))


def gen_colors(n):
    colors = []
    color = 0xFF000000
    for i in range(n):
        color ^= random.randint(0, 0xFFFFFF)
        colors.append(color)
    return colors


colors = gen_colors(num_types)

sdl2.ext.init()
window = sdl2.ext.Window("Simulations", size=(800, 600))
window.show()
renderer = sdl2.ext.renderer.Renderer(target=window)
running = True

sim_times = []
render_times = []
refresh = time.time()

while running:
    events = sdl2.ext.get_events()
    for event in events:
        if event.type == sdl2.SDL_QUIT:
            running = False
            break

    start = time.time()
    _, x_vel, y_vel, x_pos, y_pos = sim.step(
        forces, types, x_vel, y_vel, x_pos, y_pos, 0.01
    )
    end = time.time()
    sim_times.append(end - start)

    start = time.time()
    renderer.clear(0x00000000)
    x_tmp = np.zeros(x_pos.shape, dtype=np.float32)
    x_pos.get(ary=x_tmp)
    y_tmp = np.zeros(y_pos.shape, dtype=np.float32)
    y_pos.get(ary=y_tmp)

    for t, x, y in zip(types, x_tmp, y_tmp):
        xi = int(x * 40 + 400)
        yi = int(y * 30 + 300)
        sdl2.sdlgfx.filledCircleColor(renderer.sdlrenderer, xi, yi, 1, colors[t])
    renderer.present()
    window.refresh()
    end = time.time()
    render_times.append(end - start)

    if time.time() - refresh > 3:
        print(
            "Sim: {:.2f}ms, Render: {:.2f}ms, Framerate: {:.2f}/s".format(
                np.mean(sim_times) * 1000,
                np.mean(render_times) * 1000,
                1 / (np.mean(sim_times) + np.mean(render_times)),
            )
        )
        sim_times = []
        render_times = []
        refresh = time.time()
