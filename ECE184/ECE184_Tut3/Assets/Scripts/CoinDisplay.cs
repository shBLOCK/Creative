using System;
using TMPro;
using UnityEngine;

public class CoinDisplay : MonoBehaviour {
    private TMP_Text _text;

    private void Awake() {
        _text = GetComponent<TMP_Text>();
    }

    private void Start() {
        GameManager.instance.coinsChanged += (_, value) => _text.text = value.ToString();
    }
}