class_name Viewport3DCamera extends Camera3D

@export var real_transform := Transform3D.IDENTITY

@export var clip_at_viewport_plane := false
@export var clip_plane_offset := 0.0

func _update_projection():
	var viewport := get_viewport() as SubViewport
	var viewport_3d := viewport.get_child(0) as Viewport3DMarker
	
	var local_real_camera_pos := viewport_3d.real_transform.inverse() * real_transform.origin
	var screen_scale := viewport_3d.v_size / viewport_3d.real_v_size
	var local_camera_pos := local_real_camera_pos * screen_scale
	
	self.top_level = true
	self.position = viewport_3d.global_transform * local_camera_pos
	self.basis = viewport_3d.global_basis
	var z_near: float = self.near
	if clip_at_viewport_plane:
		z_near = local_camera_pos.z + clip_plane_offset
	
	self.keep_aspect = KEEP_HEIGHT
	self.set_frustum(
		viewport_3d.v_size * (z_near / local_camera_pos.z),
		-Vector2(local_camera_pos.x, local_camera_pos.y) / local_camera_pos.z * z_near,
		z_near,
		self.far 
	)

func unproject_ndc_global(pos: Vector2):
	var viewport := get_viewport()
	pos *= Vector2(self.size * (float(viewport.size.x) / viewport.size.y), self.size) * 0.5
	pos += self.frustum_offset
	return quaternion * Vector3(pos.x, pos.y, -self.near)

func _process(_delta: float) -> void:
	if is_current():
		_update_projection()
