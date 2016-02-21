subshader "B17_Wreck1_M1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.0627451 0.0627451 0.0627451;
	materialSpecularPower 12.5;
	texture "texture/B17Fu1_T";
}

subshader "B17_Wreck1_M1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.0627451 0.0627451 0.0627451;
	materialSpecularPower 12.5;
	texture "texture/B17Win_T";
}

subshader "B17_Wreck1_M1_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.0627451 0.0627451 0.0627451;
	materialSpecularPower 12.5;
	texture "texture/B17Fu2_T";
}

subshader "B17_Wreck1_M1_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1 1 1;
	texture "texture/B17Eng_T";
}

subshader "B17_Wreck1_M1_Material4" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.0627451 0.0627451 0.0627451;
	materialSpecularPower 12.5;
	transparent true;
	twosided true;
	envmap true;
	texture "texture/B17Gla_T";
}

