from helix_render import Transform, Mesh, MeshObject, Scene
from math import pi
import numpy as np

Scene1 = Scene()
path = "HelixRender/examples/cube.obj"
Cube_mesh = Scene1.create_mesh(path)
Cube_object = Scene1.add_object("cube")
Scene1.apply_mesh_to_object(Cube_object, Cube_mesh)

Scene1.translate_object(Cube_object, [0,1,0])
Scene1.rotate_object(Cube_object, [0,pi/4,0])
Scene1.scale_object(Cube_object, [1,1,1])

array = np.array(Scene1.get_object_matrix(Cube_object))
print(array.T)

