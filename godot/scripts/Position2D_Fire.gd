extends Position2D

# This script assumnes that the Rifle is the first child of the Position2D


func _ready():
	pass

func fire_bullet():
	get_child(0).call("on_shoot_gun")
