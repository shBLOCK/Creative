using System;
using Unity.Mathematics;
using UnityEngine;

public class CameraController : MonoBehaviour {
    public Transform target;
    public float horizontalOffset, verticalOffset;
    public float lerpTime = 0.5f;

    private void LateUpdate() {
        var targetCamPos = target.position - target.forward * horizontalOffset + target.up * verticalOffset;
        var camPos = transform.position;
        transform.position = targetCamPos;
        transform.LookAt(target);
        transform.position = Vector3.Lerp(camPos, targetCamPos, 1f - math.exp(-lerpTime * Time.deltaTime));
    }
}