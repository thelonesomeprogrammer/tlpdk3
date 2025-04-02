<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class ExperienceItem extends Model
{
    use HasFactory;

    public $timestamps = false;

    public function experience()
    {
        return $this->belongsTo(Experience::class);
    }
}
