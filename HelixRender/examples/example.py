from helix_render import Transform, Mesh
from math import pi
import numpy as np

def printMatrix(matrix):
    for i in range(len(matrix[0])):
        print("[", end="")
        for j in range(len(matrix)):
            print(f"{matrix[j][i]: .2f} ", end="")
        print("]")
    print()

t = Transform()
t.translate([1,0,0])
t.rotate([pi,0,0])
t.scale([2,2,2])

m = t.get_matrix()
printMatrix(m)
np_mat = np.array(m)
pos = np.array([[0,1,1,1]])

print("Position before:")
printMatrix(pos)

new_pos = np_mat.T.dot(pos.T).T

print("Position after:")
printMatrix(new_pos)

mesh = Mesh()
mesh.load_obj("HelixRender/examples/cube.obj")
print(f"Loaded mesh with {mesh.vertex_count()} vertices and {mesh.face_count()} faces.")