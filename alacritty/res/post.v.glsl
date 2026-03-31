// Fullscreen triangle vertex shader for post-processing.
// Generates a triangle that covers the entire screen using gl_VertexID.
// No vertex attributes needed — just draw 3 vertices.

out vec2 vTexCoord;

void main() {
    // Generate fullscreen triangle from vertex ID (0, 1, 2).
    // This produces coordinates that cover [-1, 1] in clip space.
    float x = float((gl_VertexID & 1) << 2) - 1.0;
    float y = float((gl_VertexID & 2) << 1) - 1.0;
    gl_Position = vec4(x, y, 0.0, 1.0);

    // Map to [0, 1] texture coordinates.
    vTexCoord = gl_Position.xy * 0.5 + 0.5;
}
