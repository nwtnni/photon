width 800
height 600
samples 1024

integrator
    light

camera
    -20.0 15.0 20.0
      0.0  2.5  0.0
      0.0  1.0  0.0
    45
    1.333333
    0.0001
    0.035

light
    quad
        -8.0 0.0   8.0
         0.0 0.0 -16.0
        16.0 0.0   0.0
        lambertian
            1.0 1.0 1.0
        1.0 1.0 1.0

surface
    mesh
        models/dragon.obj
        specular 
            0.8 0.8 1.0
            1.5
