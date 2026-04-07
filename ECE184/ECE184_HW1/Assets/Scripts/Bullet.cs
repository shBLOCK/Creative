using UnityEngine;

public class Bullet : MonoBehaviour {
    [SerializeField] private float speed = 20f;
    [SerializeField] private float ttl = 10f;

    void FixedUpdate() {
        transform.Translate(Vector3.forward * (speed * Time.fixedDeltaTime));
        
        ttl -= Time.fixedDeltaTime;
        if (ttl <= 0f) Destroy(gameObject);
    }

    private void OnTriggerEnter(Collider other) {
        if (other.GetComponent<Enemy>() != null) {
            Destroy(gameObject);
            KillUtils.Kill(other.gameObject);
            FindFirstObjectByType<PlayerController>().AddScore(1);
        }
    }
}