using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerMovement : MonoBehaviour
{
    [SerializeField] private float speed;
    [SerializeField] private float jumpSpeed;

    private Rigidbody rb;
    private void Awake() { rb = GetComponent<Rigidbody>(); }

    private void FixedUpdate()
    {
        Vector3 dir = Vector3.zero;
        if (Keyboard.current.wKey.IsPressed()) { dir += Vector3.forward; }
        if (Keyboard.current.aKey.IsPressed()) { dir += Vector3.left; }
        if (Keyboard.current.sKey.IsPressed()) { dir += Vector3.back; }
        if (Keyboard.current.dKey.IsPressed()) { dir += Vector3.right; }
        Vector3 velocity = speed * dir.normalized;
        velocity.y = rb.linearVelocity.y;
        rb.linearVelocity = velocity;
    }

    private void Update()
    {
        if (Keyboard.current.spaceKey.wasPressedThisFrame &&
            Physics.Raycast(transform.position, Vector3.down, 1.01f))
        {
            rb.AddForce(jumpSpeed * Vector3.up, ForceMode.VelocityChange);
        }
    }
}
