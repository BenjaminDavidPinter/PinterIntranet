import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { OnInit } from '@angular/core';
import { HealthInfo } from '../models/health-info.model';
import { ApiService } from './api.service';
import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, HttpClientModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit{
  constructor(private apiService: ApiService){}

  title = 'Pinter Family Website';
  health: HealthInfo | null = null;

  ngOnInit(): void {
    this.apiService.getHealth().subscribe({
      next: (response) =>  this.health = response,
      error: (err) => console.error("Api call failed", err)
    });
  }
}
