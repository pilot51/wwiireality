subshader "1P_spitfire_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.156863 0.156863 0.156863;
	materialSpecularPower 0.5;
	envmap true;
	texture "texture/spitfire_Britain_a";
}

subshader "1P_spitfire_m1_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1 1 1;
	texture "texture/1pSpitfire2_Z";
}

subshader "1P_spitfire_m1_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1 1 1;
	texture "texture/1pSpitfire_Z";
}

subshader "1P_spitfire_m1_Material4" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1 1 1;
	transparent true;
	texture "texture/Yak9_crosshair_I";
}

