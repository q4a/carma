//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#include <GL/gl.h>
#include <algorithm>
#include <utility>
#include "texturizer.h"

using raii_wrapper::file;

// Read all textures from file f.
bool texture_renderer_t::read(file& f)
{
    CHECK_READ(resource_file_t::read_file_header(f));

    pixelmap_t pmap;

    while (pmap.read(f))
    {
        texture_t* tex = new texture_t;
        if (!tex)
            return false;
        tex->pixelmap = pmap;
        std::transform(tex->pixelmap.name.begin(), tex->pixelmap.name.end(), tex->pixelmap.name.begin(), toupper);
        if (!cache.insert(std::make_pair(tex->pixelmap.name, tex)).second)
            printf("Texture %s already present in cache, not adding.\n", tex->pixelmap.name.c_str());
    }

    dump_cache();

    return true;
}

void texture_renderer_t::dump_cache()
{
    for(std::map<std::string, texture_t*>::iterator it = cache.begin(); it != cache.end(); ++it)
        printf("'%s' -> %p\n", (*it).first.c_str(), (*it).second);
}

bool texture_renderer_t::set_texture(std::string name)
{
    if (cache.find(name) == cache.end())
    {
        printf("Texture %s not found in cache!\n", name.c_str());
        dump_cache();
        return false;
    }
    texture_t* tex = cache[name];
    if (tex->bound_id)
    {
        glBindTexture(GL_TEXTURE_2D, tex->bound_id); // Set our texture handle as current
        return true;
    }

    uint16_t w = tex->pixelmap.w;
    uint16_t h = tex->pixelmap.h;

    glGenTextures(1, &(tex->bound_id));            // Allocate space for texture
    glBindTexture(GL_TEXTURE_2D, tex->bound_id); // Set our texture handle as current

    // Create the texture
    // try GL_INTENSITY or GL_INTENSITY8
    //                             v
    glTexImage2D(GL_TEXTURE_2D, 0, 1, w, h, 0, GL_COLOR_INDEX, GL_UNSIGNED_BYTE, tex->pixelmap.data);

    // Specify filtering and edge actions
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);

    return true;
}

// test draw a texture
bool texture_renderer_t::draw_texture(std::string name)
{
    glEnable(GL_TEXTURE_2D);
    if (!set_texture(name))
        return false;

    glColor3f(1.0, 1.0, 1.0);
    glBegin(GL_QUADS);
    glTexCoord2f(0, 0);
    glVertex2f(-1.0, -1.0);
    glTexCoord2f(1, 0);
    glVertex2f(1.0, -1.0);
    glTexCoord2f(1, 1);
    glVertex2f(1.0, 1.0);
    glTexCoord2f(0, 1);
    glVertex2f(-1.0, 1.0);
    glEnd();

    glDisable(GL_TEXTURE_2D);
    glBindTexture(GL_TEXTURE_2D, 0);

    return true;
}