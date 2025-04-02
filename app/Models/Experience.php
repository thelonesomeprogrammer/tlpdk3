<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;


class Experience extends Model
{

    public $timestamps = false;

    public function items()
    {
        return $this->hasMany(ExperienceItem::class);
    }

}
