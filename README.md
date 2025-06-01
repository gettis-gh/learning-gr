# learning-gr
Learning Graphic Rendering from Scratch!!!

## Commit 4: Update 0.4.0
- (Commit 2): Note: Rompi el Bitmap writer reorganizando el proyecto y que pereza reescribirlo 
- (Commit 3):
    - Note: Todavia no devuelvo el Bitmap writer
    - Note: Estuve probando a prerenderizar el color buffer y va rapido pero no se actualiza al redimensionar
    - Change: **Nueva interfaz**
        - rasterizer::{
            clear_frame, // (&mut context, color);
            draw_pixels, // (&mut context, triangle_grid, color_shader, condition);
        }
- (Commit 4): 
    - Note: implemented 3d model loading, pixel coloring presicion, backfaces culling
