subshader "rh_aichi_prop_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.6 0.6 0.6;
	materialSpecularPower 21.0;
	envmap true;
	texture "texture/Aichi1_r";
}

subshader "zero_propblur_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
	twosided true;
	texture "texture/Zero_Propblr";
}

