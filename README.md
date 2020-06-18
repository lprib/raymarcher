# raymarcher
Julia set raymarcher implemented in rust

![Output GIF](/out.gif?raw=true "Optional Title")

```
marcher-3d 
Render 3d julia set in window

USAGE:
    marcher 3d [OPTIONS] --c=<cw>,<cx>,<cy>,<cz> --height <height> --width <width>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -w, --width <width>                              width of framebuffer
    -h, --height <height>                            height of framebuffer
    -c, --c=<cw>,<cx>,<cy>,<cz>                      c value of julia set
        --aa-level <aa-level>
            level of anti-aliasing. --aa-level 2 will provide a 2x2 subpixel grid [default: 2]

        --backplane-pos=<x>,<y>,<z>
            values of x/y/z where rays will be assumed to be a miss (ie. back clipping planes) [default: 3,3,3]

        --bg-color=<r>,<g>,<b>
            normalized (each element in [0, 1]) color of background [default: 0,0,0]

        --camera-pos=<x>,<y>,<z>                     position of camera in 3d space [default: 2,4,4]
        --light-pos=<x>,<y>,<z>                      position of light in 3d space [default: 2,4,4]
        --look-at=<x>,<y>,<z>                        position to point camera towards in 3d space [default: 0,0,0]
        --object-color=<r>,<g>,<b>                   normalized ambient color of the julia set [default: 0.8,0,0]
        --specular-color=<r>,<g>,<b>                 normalized specular highlight color of the render [default: 1,1,1]
        --specular-shininess <specular-shininess>
            Phong shininess value used when calculating specular highlights [default: 50]

    -z, --zoom <zoom>                                camera zoom [default: 1]
```
