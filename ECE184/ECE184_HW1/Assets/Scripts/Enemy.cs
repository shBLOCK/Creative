using UnityEngine;

public class Enemy : MonoBehaviour {
    [SerializeField] private float speed = 5f;
    
    private GameObject player;

    void Start() {
        player = GameObject.Find("Player");
    }

    void Update() {
        if (player) {
            transform.LookAt(new Vector3(player.transform.position.x, transform.position.y, player.transform.position.z));
            transform.Translate(Vector3.forward * (Time.deltaTime * speed));
        }
    }

    private void OnTriggerEnter(Collider other) {
        if (other.gameObject == player) {
            Destroy(gameObject);
            KillUtils.Kill(other.gameObject);
        }
    }
}