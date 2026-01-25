import numpy as np
from helix_render import Engine


def print_matrix(name, matrix):
    print(f"\n--- {name} ---")
    print(np.round(np.array(matrix), 2))


# 1. Initialization
engine = Engine()
sm = engine.scene_manager
rm = engine.resource_manager
scene = sm.active_scene

# 2. Resource Loading
# Ensure you have a valid .obj file at this path
cube_mesh = rm.load_mesh("examples/cube.obj")

# 3. Basic Object Creation & Properties
print(">>> Testing Basic Object Creation")
cube = scene.create_mesh_object("ParentCube", cube_mesh, None)
name = scene.obj_name(cube)
print(f"Created Object Name: {name}")

# 4. Testing Position & Translation
print("\n>>> Testing Translation")
scene.obj_set_pos(cube, [1.0, 2.0, 3.0])
print_matrix("After Set Pos [1, 2, 3]", scene.obj_transform(cube))

scene.obj_translate(cube, [10.0, 0.0, -1.0])
print_matrix("After Translate [10, 0, -1]", scene.obj_transform(cube))

# 5. Testing Rotation & Scale
print("\n>>> Testing Rotation & Scale")
# Rotate 90 degrees (approx 1.57 rad) around Y axis
scene.obj_set_rotatation(cube, [0.0, 1.5708, 0.0])
scene.obj_set_scale(cube, [2.0, 2.0, 2.0])
print_matrix("After 90deg Y-Rot & 2x Scale", scene.obj_transform(cube))
