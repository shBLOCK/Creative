extends SubViewport


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var viewport := get_viewport() as SubViewport
	var mouse_ndc := ((viewport.get_mouse_position() / Vector2(viewport.size)) - Vector2(0.5, 0.5)) * Vector2(2, -2)
	var mouse_ray: Vector3 = get_viewport().get_camera_3d().unproject_ndc_global(mouse_ndc)
	
	%MouseRay.position = $Camera.position
	%MouseRay.target_position = mouse_ray.normalized() * 1e3

func _unhandled_input(event: InputEvent) -> void:
	var collider = %MouseRay.get_collider()
	if collider is Area3D:
		collider.input_event.emit(get_camera_3d(), event, %MouseRay.get_collision_point(), %MouseRay.get_collision_normal(), %MouseRay.get_collider_shape())
