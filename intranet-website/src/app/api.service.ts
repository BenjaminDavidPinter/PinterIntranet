import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { HealthInfo } from '../models/health-info.model';

@Injectable({providedIn: 'root'})
export class ApiService {

  private healthRoute = '/api/health';

  constructor(private http: HttpClient) { }

  getHealth(): Observable<HealthInfo> {
    return this.http.get<HealthInfo>(this.healthRoute);
  }
}
