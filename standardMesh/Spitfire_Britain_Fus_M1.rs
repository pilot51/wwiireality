subshader "spitfire_fus_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.156863 0.156863 0.156863;
	materialSpecularPower 0.5;
	envmap true;
	texture "texture/spitfire_Britain_a";
}

subshader "spitfire_fus_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.160784 0.160784 0.160784;
	materialSpecularPower 0.5;
	transparent true;
	twosided true;
	envmap true;
	texture "texture/Spitfire_Alpha_a";
}

