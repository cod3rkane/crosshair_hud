#version 330 core

in vec2 TextureCoord;

// texture sampler
uniform sampler2D texture1;

out vec4 FragColor;

void main() {
     FragColor = texture(texture1, TextureCoord);
     // FragColor = vec4(0.2, 1.0, 0.5, 1.0);
}
