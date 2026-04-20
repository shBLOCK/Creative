using System;
using UnityEngine;

public class Coin : MonoBehaviour {
    public int value;

    private void OnTriggerEnter(Collider other) {
        if (other.CompareTag("Player")) {
            GameManager.instance.CollectCoin(value);
            Destroy(gameObject);
        }
    }
}