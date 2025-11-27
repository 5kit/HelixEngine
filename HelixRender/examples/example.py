from helix_render import Transform, Mesh, MeshObject, Scene
from math import pi
import numpy as np

Scene1 = Scene()
path = "HelixRender/examples/cube.obj"
Scene1.add_object("cube", path)

obj_handle = Scene1.get_object("cube")
Scene1.translate_object(obj_handle, [0,1,0])
Scene1.rotate_object(obj_handle, [0,pi/4,0])
Scene1.scale_object(obj_handle, [1,1,1])

array = np.array(Scene1.get_object_matrix(obj_handle))
print(array.T)

