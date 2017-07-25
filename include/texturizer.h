//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#include <OpenGL/gl.h>
#include "blocks.h"
#include <map>

class texture_t
{
public:
    GLuint bound_id;
    pixelmap_t pixelmap;

    texture_t() : bound_id(0), pixelmap() {}

    inline void dump() { pixelmap.dump(); }
};

class texture_renderer_t
{
public:
    std::map<std::string, texture_t*> cache;
    // Pixel map tables for GL.
    GLfloat* alpha_tab {nullptr};
    GLfloat* r_tab {nullptr};
    GLfloat* g_tab {nullptr};
    GLfloat* b_tab {nullptr};

    texture_renderer_t() = default;
    ~texture_renderer_t();

    bool read(raii_wrapper::file& f);
    bool set_texture(std::string name);
    void reset_texture();
    bool draw_texture(std::string name);
    void dump_cache();
    void dump_cache_textures();

    /* Set palette for converting GL_COLOR_INDEX pixmaps to textures. */
    bool set_palette(pixelmap_t palette);
};