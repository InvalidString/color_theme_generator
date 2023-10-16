#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;

// NOTE: Add here your custom variables

void main()
{
    float PI = 3.1415;

    vec2 uv = fragTexCoord - vec2(.5, .5);

    float r = dot(uv,uv);

    float angle = atan(uv.y , uv.x) / (2*PI);
    vec4 color = texture(texture0, vec2(angle, 0));

    if(r < 0.25 && r > 0.1){
        finalColor = color;
    }else{
        finalColor = vec4(0);
    }

}
