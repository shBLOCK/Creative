using System;
using UnityEngine;

public class CoinDoor : MonoBehaviour {
    public int coinsRequired = 10;

    private Door _door;
    
    private void Awake() {
        _door = GetComponent<Door>();
    }

    private void Start() {
        GameManager.instance.coinsChanged += (_, value) => _door.SetOpen(value >= coinsRequired);
    }
}