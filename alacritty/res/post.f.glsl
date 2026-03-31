#define PI 3.14159265359

in vec2 vTexCoord;

uniform sampler2D sceneTex;
uniform vec2 resolution;
uniform float cellHeight;
uniform float scanlineIntensity;
uniform float glowIntensity;
uniform float scanlineThickness;
uniform float scanlineSpacing;

layout(location = 0) out vec4 fragColor;

float Gaussian2D(float x, float y, float sigma) {
    return exp(-0.5 * (x * x + y * y) / (sigma * sigma));
}

void main() {
    vec4 original = texture(sceneTex, vTexCoord);
    vec2 texel = 1.0 / resolution;

    float sigma = max(cellHeight * 0.3, 2.5);

    vec3 blurred = vec3(0.0);
    float totalWeight = 0.0;
    for (int x = -10; x <= 10; x++) {
        for (int y = -10; y <= 10; y++) {
            float weight = Gaussian2D(float(x), float(y), sigma);
            if (weight < 0.001) continue;
            vec2 offset = vec2(float(x), float(y)) * texel;
            vec3 s = texture(sceneTex, vTexCoord + offset).rgb;
            float lum = dot(s, vec3(0.2126, 0.7152, 0.0722));
            blurred += s * weight * smoothstep(0.05, 0.3, lum);
            totalWeight += weight;
        }
    }
    blurred /= totalWeight;

    vec3 glow = blurred * glowIntensity * 2.0;

    float yPixel = floor(resolution.y - gl_FragCoord.y);
    float scanPeriod = max(scanlineThickness + scanlineSpacing, 2.0);
    float posInPeriod = mod(yPixel, scanPeriod);
    float scanline = 1.0 - scanlineIntensity * step(scanPeriod - scanlineThickness, posInPeriod);
    vec3 color = original.rgb * scanline;

    // Glow partially bleeds through scanlines — attenuated on dark bands.
    vec3 attenuatedGlow = glow * mix(1.0, 0.35, 1.0 - scanline);
    color = 1.0 - (1.0 - color) * (1.0 - attenuatedGlow);

    fragColor = vec4(color, original.a);
}
