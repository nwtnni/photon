Measurements collected by rendering `scenes/` directory with `hyperfine`.

- Baseline

| Scene               | Time     |
|---------------------|----------|
| area-light.txt      |  13.418s |
| buddha.txt          |   3.438s |
| cornell-box.txt     |  31.349s |
| sphere.txt          |   2.222s |

- Static dispatch

| Scene               | Time     |
|---------------------|----------|
| area-light.txt      |  13.289s |
| buddha.txt          |   3.401s |
| cornell-box.txt     |  31.108s |
| sphere.txt          |   2.810s |
