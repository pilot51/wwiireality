subshader "pak40_mount_wreck_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.266667 0.192157 0.545098;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 12.5;
	texture "texture/pak40_shield_dest_J";
}

subshader "pak40_mount_wreck_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.2 0.552941 0.176471;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 12.5;
	texture "texture/pak40_gun_dest_J";
}

subshader "pak40_mount_wreck_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0 0 0;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 12.5;
	transparent true;
	twosided true;
	texture "texture/pak40_alphas_dest_J";
}

